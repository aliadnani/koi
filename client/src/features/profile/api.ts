import ky from "ky";
import { Globals } from "../../common/globals";
import { bearerAuthHeader, camelCaseObject } from "../../common/utils";
import { UserProfileWithProjects } from "../../interfaces/profile";

async function getUserProfile(token?: string): Promise<UserProfileWithProjects> {
  const response = await ky.get(`${Globals.apiBaseUrl}/profile`, {
    headers: { Authorization: bearerAuthHeader(token ?? "") },
  });

  return camelCaseObject(await response.json());
}

export { getUserProfile };
