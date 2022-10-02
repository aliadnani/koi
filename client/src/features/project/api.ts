import ky from "ky";
import { Globals } from "../../common/globals";
import {
  bearerAuthHeader,
  camelCaseObject,
  snakeCaseObject,
} from "../../common/utils";
import { Feedback, FeedbackApi } from "../../interfaces/feedback";
import { UserProfile, UserProfileApi } from "../../interfaces/profile";
import {
  NewProjectApi,
  Project,
  UserProjectAdditionRemoval,
} from "../../interfaces/project";

async function createProject(
  projectName: string,
  token: string
): Promise<Project> {
  const requestBody: NewProjectApi = snakeCaseObject({ name: projectName });

  const response = await ky.post(`${Globals.apiBaseUrl}/projects`, {
    json: requestBody,
    headers: { Authorization: bearerAuthHeader(token) },
  });

  return camelCaseObject(await response.json());
}

async function getProject(projectId: string, token: string): Promise<Project> {
  const response = await ky.get(`${Globals.apiBaseUrl}/projects/${projectId}`, {
    headers: { Authorization: bearerAuthHeader(token) },
  });

  return camelCaseObject(await response.json());
}

async function getProjectUsers(
  projectId: string,
  token: string
): Promise<UserProfile[]> {
  const response = await ky.get(
    `${Globals.apiBaseUrl}/projects/${projectId}/users`,
    {
      headers: { Authorization: bearerAuthHeader(token) },
    }
  );

  const userProfileArr: UserProfileApi[] = await response.json();

  return userProfileArr.map((f) => camelCaseObject(f));
}

async function addUserToProject(
  projectId: string,
  userToAdd: UserProjectAdditionRemoval,
  token: string
): Promise<void> {
  const response = await ky.post(
    `${Globals.apiBaseUrl}/projects/${projectId}/users`,
    {
      headers: { Authorization: bearerAuthHeader(token) },
      json: userToAdd,
    }
  );

  await response.json();
}

async function removeUserFromProject(
  projectId: string,
  userToRemove: UserProjectAdditionRemoval,
  token: string
): Promise<void> {
  const response = await ky.delete(
    `${Globals.apiBaseUrl}/projects/${projectId}/users`,
    {
      searchParams: userToRemove,
      headers: { Authorization: bearerAuthHeader(token) },
    }
  );

  await response.json();
}

async function getProjectFeedback(
  projectId: string,
  token: string
): Promise<Feedback[]> {
  const response = await ky.get(
    `${Globals.apiBaseUrl}/projects/${projectId}/feedback`,
    {
      headers: { Authorization: bearerAuthHeader(token) },
    }
  );

  const feedbackArr: FeedbackApi[] = await response.json();

  return feedbackArr.map((f) => camelCaseObject(f));
}

export {
  createProject,
  getProjectFeedback,
  getProject,
  addUserToProject,
  getProjectUsers,
  removeUserFromProject,
};
