//! The few WMI calls 2VX makes, each on a thread of its own so COM starts and ends with the work.

use windows::Win32::Foundation::E_FAIL;
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx,
    CoSetProxyBlanket, CoUninitialize, EOAC_NONE, RPC_C_AUTHN_LEVEL_CALL,
    RPC_C_IMP_LEVEL_IMPERSONATE,
};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::System::Wmi::{
    IWbemClassObject, IWbemLocator, IWbemServices, WBEM_FLAG_FORWARD_ONLY,
    WBEM_FLAG_RETURN_IMMEDIATELY, WBEM_FLAG_RETURN_WBEM_COMPLETE, WbemLocator,
};
use windows::core::{BSTR, HSTRING, PCWSTR, Result as WinResult, w};

const RPC_C_AUTHN_WINNT: u32 = 10;
const RPC_C_AUTHZ_NONE: u32 = 0;
/// How long one row may take. A busy provider ends the list rather than hanging the page.
const ROW_TIMEOUT_MS: i32 = 20_000;

/// Runs `work` on a new thread, in the multithreaded apartment.
pub fn on_com_thread<T: Send>(work: impl FnOnce() -> WinResult<T> + Send) -> WinResult<T> {
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                // SAFETY: this thread is new, and paired with CoUninitialize below when it succeeds.
                let com = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
                let result = work();
                if com.is_ok() {
                    // SAFETY: pairs with the call above, and `work` dropped its interfaces.
                    unsafe { CoUninitialize() };
                }
                result
            })
            .join()
    })
    .unwrap_or_else(|_| Err(E_FAIL.into()))
}

pub struct Wmi(IWbemServices);

impl Wmi {
    pub fn open(namespace: &str) -> WinResult<Self> {
        // SAFETY: plain COM calls on interfaces this function owns.
        unsafe {
            let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)?;
            let none = BSTR::new();
            let services = locator.ConnectServer(
                &BSTR::from(namespace),
                &none,
                &none,
                &none,
                0,
                &none,
                None,
            )?;
            CoSetProxyBlanket(
                &services,
                RPC_C_AUTHN_WINNT,
                RPC_C_AUTHZ_NONE,
                PCWSTR::null(),
                RPC_C_AUTHN_LEVEL_CALL,
                RPC_C_IMP_LEVEL_IMPERSONATE,
                None,
                EOAC_NONE,
            )?;
            Ok(Self(services))
        }
    }

    pub fn rows(&self, query: &str) -> WinResult<Vec<IWbemClassObject>> {
        // SAFETY: the out pointers are valid for each call, and the interfaces are owned here.
        unsafe {
            let rows = self.0.ExecQuery(
                &BSTR::from("WQL"),
                &BSTR::from(query),
                WBEM_FLAG_FORWARD_ONLY | WBEM_FLAG_RETURN_IMMEDIATELY,
                None,
            )?;
            let mut found = Vec::new();
            loop {
                let mut row: [Option<IWbemClassObject>; 1] = [None];
                let mut returned = 0;
                let _ = rows.Next(ROW_TIMEOUT_MS, &mut row, &mut returned);
                let [Some(row)] = row else {
                    return Ok(found);
                };
                found.push(row);
            }
        }
    }

    /// Calls a method and gives back its `ReturnValue`. `object` is the class itself for a static
    /// method, or an instance's `__PATH`. Inputs left out take the method's defaults.
    pub fn call(
        &self,
        class: &str,
        object: &str,
        method: &str,
        inputs: &[(&str, VARIANT)],
    ) -> WinResult<i32> {
        let name = HSTRING::from(method);
        // SAFETY: every out pointer is valid for its call, and the strings outlive the calls.
        unsafe {
            let mut definition = None;
            self.0.GetObject(
                &BSTR::from(class),
                WBEM_FLAG_RETURN_WBEM_COMPLETE,
                None,
                Some(&mut definition),
                None,
            )?;
            let definition: IWbemClassObject = definition.ok_or(E_FAIL)?;
            let mut signature: Option<IWbemClassObject> = None;
            let mut outputs = None;
            definition.GetMethod(PCWSTR(name.as_ptr()), 0, &mut signature, &mut outputs)?;
            let arguments = signature.map(|s| s.SpawnInstance(0)).transpose()?;
            if let Some(arguments) = &arguments {
                for (input, value) in inputs {
                    let input = HSTRING::from(*input);
                    arguments.Put(PCWSTR(input.as_ptr()), 0, value, 0)?;
                }
            }
            let mut result = None;
            self.0.ExecMethod(
                &BSTR::from(object),
                &BSTR::from(method),
                WBEM_FLAG_RETURN_WBEM_COMPLETE,
                None,
                arguments.as_ref(),
                Some(&mut result),
                None,
            )?;
            let result: IWbemClassObject = result.ok_or(E_FAIL)?;
            let mut value = VARIANT::default();
            result.Get(w!("ReturnValue"), 0, &mut value, None, None)?;
            i32::try_from(&value)
        }
    }
}

fn get(row: &IWbemClassObject, name: &str) -> Option<VARIANT> {
    let name = HSTRING::from(name);
    let mut value = VARIANT::default();
    // SAFETY: `row` is a live object and `name` is NUL-terminated.
    unsafe { row.Get(PCWSTR(name.as_ptr()), 0, &mut value, None, None) }.ok()?;
    Some(value)
}

pub fn text(row: &IWbemClassObject, name: &str) -> Option<String> {
    let value = get(row, name)?;
    BSTR::try_from(&value)
        .ok()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
}

pub fn number(row: &IWbemClassObject, name: &str) -> Option<i64> {
    i64::try_from(&get(row, name)?).ok()
}
