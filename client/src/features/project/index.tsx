import { Box, Container, Stack } from "@chakra-ui/react";
import { useQuery } from "@tanstack/react-query";
import { motion } from "framer-motion";
import { useEffect } from "react";
import { Outlet, useNavigate } from "react-router-dom";
import { Globals } from "../../common/globals";
import { FeedbackInfoBlock } from "./components/feedback-info-block";
import { FeedbackTable } from "./components/feedback-table/feedback-table";
import { Header } from "./components/header/header";
import { useSession } from "../../state/session";
import { getUserProfile } from "../profile/api";
import { useAppParams } from "../../router";

function ProjectPage() {
  // Zustand
  const { sessionToken } = useSession();

  const { projectId } = useAppParams();

  // React Router
  const navigate = useNavigate();

  // Profile from react query
  const { data: userProfile } = useQuery(
    ["profile"],
    async () => await getUserProfile(sessionToken),
    { enabled: !!sessionToken }
  );

  useEffect(() => {
    if (!sessionToken) {
      navigate("/");
    }
  }, [sessionToken]);

  useEffect(() => {
    if (userProfile?.projects[0] && !projectId) {
      navigate(userProfile.projects[0].id);
    }
  }, [userProfile]);

  return (
    <Box>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.5 }}
      >
        <Container maxW={"6xl"}>
          <Stack as={Box} spacing={{ base: 4, md: 8 }} py={{ base: 8, md: 8 }}>
            <Header />
            <Outlet />
          </Stack>
        </Container>
      </motion.div>
    </Box>
  );
}

export { ProjectPage };
