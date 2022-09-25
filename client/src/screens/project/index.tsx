import { Box, Container, Fade, Stack } from "@chakra-ui/react";
import { FeedbackTable } from "../../components/feedback-table";
import { Header } from "../../components/header";

function ProjectPage() {
  return (
    <Box>
      <Fade>
        <Container maxW={"6xl"}>
          <Stack as={Box} spacing={{ base: 4, md: 8 }} py={{ base: 8, md: 8 }}>
            <Header />
            <FeedbackTable
              feedbackItems={[
                {
                  id: "34g1g1h",
                  content: "This is test number 1!",
                  createdAt: "2022-09-35T08:57:28.429Z",
                  metadata: {},
                  type: "issue",
                },
                {
                  id: "W34HE3Q1H",
                  content: "Hi can you please do ...",
                  createdAt: "2022-09-23T03:27:28.429Z",
                  metadata: {},
                  type: "idea",
                },
                {
                  id: "32Yqe",
                  content: "IDK what i want lol",
                  createdAt: "2022-09-21T04:57:28.429Z",
                  metadata: {},
                  type: "other",
                },
              ]}
            ></FeedbackTable>
          </Stack>
        </Container>
      </Fade>
    </Box>
  );
}

export { ProjectPage };
