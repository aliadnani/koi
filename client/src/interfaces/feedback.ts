import type { CamelCasedPropertiesDeep } from "type-fest";

export type FeedbackCategory = "Idea" | "Issue" | "Other";
export type FeedbackStatus = "Default" | "Archived";

export interface FeedbackMetadataApi {
  created_at: string;
  device: string;
}

export type NewFeedback = CamelCasedPropertiesDeep<NewFeedbackApi>;

export interface NewFeedbackApi {
  additional_attributes: {};
  category: FeedbackCategory;
  description: string;
  location: string;
  project_id: string;
}

export type Feedback = CamelCasedPropertiesDeep<FeedbackApi>;

export interface FeedbackApi {
  id: string;
  additional_attributes: {};
  category: FeedbackCategory;
  status: FeedbackStatus;
  metadata: FeedbackMetadataApi;
  description: string;
  location: string;
  project_id: string;
}
