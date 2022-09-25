import ky from "ky";
import { Globals } from "../../common/globals";
import { bearerAuthHeader } from "../../common/utils";
import { UserProfile } from "../../interfaces/profile";

async function getUserProfile(token?: string): Promise<UserProfile> {
  const response = await ky.get(`${Globals.baseUrl}/profile`, {
    headers: { Authorization: bearerAuthHeader(token ?? "") },
  });

  return await response.json();
}

export { getUserProfile };
