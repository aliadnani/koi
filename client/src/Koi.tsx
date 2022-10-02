import { Box } from "@chakra-ui/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { LandingPage } from "./features/landing/landing";
import { ProjectPage } from "./features/project";
import { FeedbackTable } from "./features/project/components/feedback-table/feedback-table";
import { ProjectHelpPage } from "./features/project/components/help-page/help-page";
import { ProjectSettings } from "./features/project/components/project-settings/project-settings";

const router = createBrowserRouter([
  {
    path: "/",
    element: <LandingPage />,
  },
  {
    path: "/app",
    element: <ProjectPage />,
    children: [
      {
        path: ":projectId",
        element: <FeedbackTable />,
      },
      {
        path: ":projectId/help",
        element: <ProjectHelpPage />,
      },
      {
        path: ":projectId/settings",
        element: <ProjectSettings />,
      },
    ],
  },
]);

function Koi(): JSX.Element {
  const queryClient = new QueryClient();

  return (
    <Box>
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
      </QueryClientProvider>
    </Box>
  );
}

export default Koi;
