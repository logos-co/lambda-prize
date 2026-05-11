import React from "react";
import { Link } from "react-router-dom";

interface State {
  hasError: boolean;
  error: Error | null;
  info: string | null;
}

export class ErrorBoundary extends React.Component<
  { children: React.ReactNode },
  State
> {
  constructor(props: { children: React.ReactNode }) {
    super(props);
    this.state = { hasError: false, error: null, info: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error, info: null };
  }

  componentDidCatch(error: Error, info: React.ErrorInfo) {
    console.error("[ErrorBoundary]", error, info.componentStack);
    this.setState({ info: info.componentStack?.split("\n")[1]?.trim() ?? null });
  }

  handleReset = () => {
    this.setState({ hasError: false, error: null, info: null });
  };

  render() {
    if (!this.state.hasError) return this.props.children;

    return (
      <div className="relative min-h-screen overflow-hidden bg-slate-950 text-white">
        <div className="pointer-events-none absolute inset-0 -z-10 bg-[radial-gradient(circle_at_top,rgba(239,68,68,0.12),transparent_35%)]" />
        <div className="flex min-h-screen items-center justify-center px-6 py-20">
          <div className="w-full max-w-lg">
            <div className="rounded-[2rem] border border-rose-500/30 bg-rose-500/[0.07] p-8">
              <div className="mb-5 inline-flex h-14 w-14 items-center justify-center rounded-2xl bg-rose-500/15 text-rose-300 ring-1 ring-rose-500/20">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth={1.5}
                  className="h-7 w-7"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z"
                  />
                </svg>
              </div>
              <h1 className="text-2xl font-semibold text-white">
                Something went wrong
              </h1>
              <p className="mt-3 text-sm leading-7 text-rose-100/80">
                {this.state.error?.message ?? "An unexpected error occurred."}
              </p>
              {this.state.info ? (
                <p className="mt-2 font-mono text-xs text-slate-500">
                  {this.state.info}
                </p>
              ) : null}
              <div className="mt-7 flex flex-wrap gap-3">
                <Link
                  to="/"
                  onClick={this.handleReset}
                  className="inline-flex items-center gap-2 rounded-full bg-white px-5 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100"
                >
                  Go home
                </Link>
                <button
                  onClick={this.handleReset}
                  className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-5 py-2 text-sm text-slate-200 transition hover:bg-white/10"
                >
                  Try again
                </button>
              </div>
            </div>

            <p className="mt-6 text-center text-xs text-slate-600">
              The error has been logged to the console.
            </p>
          </div>
        </div>
      </div>
    );
  }
}
