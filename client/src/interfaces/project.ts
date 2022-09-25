import type { CamelCasedPropertiesDeep } from "type-fest";

export type Project = CamelCasedPropertiesDeep<ProjectApi>;
export interface ProjectApi {
  created_at: string;
  id: string;
  name: string;
}

export type NewProject = CamelCasedPropertiesDeep<NewProjectApi>;
export interface NewProjectApi {
  name: string;
}

export type UserProjectAdditionRemoval =
  CamelCasedPropertiesDeep<UserProjectAdditionRemovalApi>;
export interface UserProjectAdditionRemovalApi {
  name: string;
}
