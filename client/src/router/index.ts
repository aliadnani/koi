import { useParams } from "react-router-dom";

export interface routeParams {
  projectId: string;
}

const useAppParams = () => useParams<keyof routeParams>();

export { useAppParams };
