import { useState } from "react";
import { toast } from "sonner";
import { refreshRestorePoints } from "@/hooks/use-system";
import { backend, toBackendError } from "@/services/backend";

/** Windows may skip a point it thinks is too soon after the last, so that's said rather than hidden. */
export function useMakeRestorePoint() {
  const [making, setMaking] = useState(false);

  const make = async () => {
    setMaking(true);
    try {
      const { created } = await backend.createRestorePoint();
      if (created) toast.success("Restore point made");
      else {
        toast.info("Windows didn't make a new one", {
          description: "It skips a new point when it made one recently.",
        });
      }
    } catch (err) {
      toast.error(toBackendError(err).message);
    } finally {
      refreshRestorePoints();
      setMaking(false);
    }
  };

  return { making, make };
}
