import ky from "ky";
import { Globals } from "../../common/globals";
import { basicAuthHeader, snakeCaseObject } from "../../common/utils";
import { NewUserProfileApi, SessionApi } from "../../interfaces/profile";

async function registerAccount(
  name: string,
  email: string,
  password: string
): Promise<SessionApi> {
  const requestBody: NewUserProfileApi = snakeCaseObject({
    name,
    email,
    password,
  });

  const response = await ky.post(`${Globals.apiBaseUrl}/register`, {
    json: requestBody,
  });
  return await response.json();
}

async function logIn(email: string, password: string): Promise<SessionApi> {
  const response = await ky.post(`${Globals.apiBaseUrl}/login`, {
    headers: { Authorization: basicAuthHeader(email, password) },
  });

  if (response.status === 403) {
    throw new Error("Oh no!");
  }

  return await response.json();
}

export { registerAccount, logIn };
