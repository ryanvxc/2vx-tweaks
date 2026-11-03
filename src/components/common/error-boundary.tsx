import { Component, type ErrorInfo, type ReactNode } from "react";
import { logger } from "@/services/logger";
import { ErrorState } from "./states";

interface State {
  error: Error | null;
}

/** Catches render errors in a page so one broken view can't take down the shell. */
export class ErrorBoundary extends Component<{ children: ReactNode }, State> {
  state: State = { error: null };

  static getDerivedStateFromError(error: Error): State {
    return { error };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    logger.error(`${error.message}\n${info.componentStack ?? ""}`, "react");
  }

  render() {
    if (this.state.error) {
      return (
        <div className="p-8">
          <ErrorState
            title="This page failed to render"
            message={this.state.error.message}
            onRetry={() => this.setState({ error: null })}
          />
        </div>
      );
    }
    return this.props.children;
  }
}
