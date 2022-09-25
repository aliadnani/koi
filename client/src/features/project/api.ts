import ky from "ky";
import { Globals } from "../../common/globals";
import {
  bearerAuthHeader,
  camelCaseObject,
  snakeCaseObject,
} from "../../common/utils";
import { Feedback, FeedbackApi } from "../../interfaces/feedback";
import { NewProjectApi, Project } from "../../interfaces/project";

async function createProject(
  projectName: string,
  token: string
): Promise<Project> {
  const requestBody: NewProjectApi = snakeCaseObject({ name: projectName });

  const response = await ky.post(`${Globals.baseUrl}/projects`, {
    json: requestBody,
    headers: { Authorization: bearerAuthHeader(token) },
  });

  return camelCaseObject(await response.json());
}



async function getProjectFeedback(
  projectId: string,
  token: string
): Promise<Feedback[]> {
  const response = await ky.get(
    `${Globals.baseUrl}/projects/${projectId}/feedback`,
    {
      headers: { Authorization: bearerAuthHeader(token) },
    }
  );

  const feedbackArr: FeedbackApi[] = await response.json();

  return feedbackArr.map((f) => camelCaseObject(f));
}


export { createProject, getProjectFeedback };