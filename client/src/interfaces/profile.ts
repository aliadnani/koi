import type { CamelCasedPropertiesDeep } from "type-fest";
import { ProjectApi } from "./project";

export type UserProfile = CamelCasedPropertiesDeep<UserProfileApi>;
export interface UserProfileApi {
  id: string;
  name: string;
  email: string;
  created_at: string;
}
export type UserProfileWithProjects =
  CamelCasedPropertiesDeep<UserProfileWithProjectsApi>;
export interface UserProfileWithProjectsApi {
  user_profile: UserProfileApi;
  projects: ProjectApi[];
}

export type NewUserProfile = CamelCasedPropertiesDeep<NewUserProfileApi>;
export interface NewUserProfileApi {
  email: string;
  name: string;
  password: string;
}

export type Session = CamelCasedPropertiesDeep<SessionApi>;
export interface SessionApi {
  token: string;
  user_profile: UserProfileApi;
}
