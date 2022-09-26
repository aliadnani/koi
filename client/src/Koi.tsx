import { Box } from "@chakra-ui/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { LandingPage } from "./features/landing/landing";
import { ProjectPage } from "./features/project";

const router = createBrowserRouter([
  {
    path: "/",
    element: <LandingPage />,
  },
  {
    path: "/app",
    element: <ProjectPage />,
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
