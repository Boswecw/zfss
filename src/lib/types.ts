/**
 * ZFSS TypeScript Types
 *
 * Types matching the Rust models for frontend use.
 */

// ID types (string aliases for clarity)
export type SignalId = `sig_${string}`;
export type IssueId = `iss_${string}`;
export type DecisionId = `dec_${string}`;
export type ArtifactId = `art_${string}`;
export type ResponseId = `rsp_${string}`;
export type AttachmentId = `att_${string}`;

// Signal types
export type SignalSource =
  | "in_app"
  | "email"
  | "dm"
  | "call"
  | "internal"
  | "partner"
  | "monitoring";

export type SignalStatus =
  | "new"
  | "linked"
  | "needs_info"
  | "responded"
  | "closed";

export interface SignalEnvironment {
  os?: string;
  device?: string;
  browser?: string;
  locale?: string;
}

export interface SignalReporter {
  external_user_id?: string;
  contact?: string;
}

export interface Signal {
  id: SignalId;
  source: SignalSource;
  raw_text: string;
  app_key?: string;
  app_version?: string;
  environment?: SignalEnvironment;
  reporter?: SignalReporter;
  status: SignalStatus;
  linked_issue_id?: IssueId;
  created_at: string;
  created_by: string;
}

export interface SignalCreate {
  source: SignalSource;
  raw_text: string;
  app_key?: string;
  app_version?: string;
  environment?: SignalEnvironment;
  reporter?: SignalReporter;
}

// Issue types
export type Classification = "Bug" | "UX" | "Feature" | "Limitation";
export type Severity = "blocker" | "major" | "minor" | "idea";
export type Frequency = "one" | "some" | "many";
export type IssueStatus =
  | "pending_decision"
  | "decided"
  | "in_progress"
  | "ready_for_verification"
  | "closed";

export interface Issue {
  id: IssueId;
  title: string;
  description?: string;
  classification: Classification;
  severity: Severity;
  frequency: number;
  status: IssueStatus;
  close_requires_artifact: boolean;
  created_at: string;
  created_by: string;
}

export interface IssueCreate {
  title: string;
  description?: string;
  classification: Classification;
  severity: Severity;
}

// Decision types
export type DecisionType =
  | "FixNow"
  | "FixLater"
  | "DocumentClarify"
  | "WontFix"
  | "DeEscalate";

export interface Decision {
  id: DecisionId;
  issue_id: IssueId;
  decision_type: DecisionType;
  rationale: string;
  decided_by: string;
  decided_at: string;
  steward_deadline_days: number;
  supersedes_id?: DecisionId;
}

export interface DecisionCreate {
  issue_id: IssueId;
  decision_type: DecisionType;
  rationale: string;
  steward_deadline_days?: number;
}

// Artifact types
export type ArtifactType = "Code" | "Logic" | "Knowledge" | "Test" | "Law";

export interface Artifact {
  id: ArtifactId;
  issue_id: IssueId;
  artifact_type: ArtifactType;
  title: string;
  description?: string;
  ref_url?: string;
  note?: string;
  verified: boolean;
  verified_by?: string;
  verified_at?: string;
  created_at: string;
  created_by: string;
}

export interface ArtifactCreate {
  issue_id: IssueId;
  artifact_type: ArtifactType;
  title: string;
  description?: string;
  ref_url?: string;
  note?: string;
}

// Response types
export type ResponseChannel = "email" | "in_app" | "dm" | "phone" | "other";
export type ApprovalState =
  | "draft"
  | "pending"
  | "approved"
  | "sent"
  | "blocked";

export interface Response {
  id: ResponseId;
  signal_id: SignalId;
  issue_id?: IssueId;
  response_class: string;
  channel: ResponseChannel;
  body: string;
  approval_state: ApprovalState;
  policy_violations: string[];
  drafted_by: string;
  drafted_at: string;
  approved_by?: string;
  approved_at?: string;
  sent_at?: string;
  blocked_reason?: string;
}

export interface ResponseCreate {
  signal_id: SignalId;
  issue_id?: IssueId;
  response_class: string;
  channel: ResponseChannel;
  body: string;
}

// User types
export type UserRole = "Steward" | "Operator" | "Engineer" | "AI";

export interface User {
  id: string;
  display_name: string;
  role: UserRole;
  is_active: boolean;
  created_at: string;
}

export interface CurrentUser {
  id: string;
  display_name: string;
  role: UserRole;
}
