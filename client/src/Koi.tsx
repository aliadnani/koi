import { Box, Heading, Text } from "@chakra-ui/react";
import { FeedbackTable } from "./features/dashboard/feedback-table";
import { ProjectSelector } from "./features/dashboard/project-selector";

function Koi(): JSX.Element {
  return (
    <Box maxW="1200px" margin="0 auto">
      <Box textAlign="center" className="koi-info" padding="1rem">
        <Text>Koi - a no-frills feedback platform</Text>
      </Box>
      <Box textAlign="center" className="feedback-overview">
        <Heading>Feedback!</Heading>
        <ProjectSelector
          projects={[
            { id: "1", name: "project1" },
            { id: "2", name: "project2" },
            { id: "3", name: "project3" },
          ]}
          selectedProjectId={"2"}
        ></ProjectSelector>
        <FeedbackTable
          feedbackItems={[
            {
              id: "34g1g1h",
              content: "This is test number 1!",
              createdAt: "",
              metadata: {},
              type: "issue",
            },
            {
              id: "W34HE3Q1H",
              content: "Hi can you please do ...",
              createdAt: "",
              metadata: {},
              type: "idea",
            },
            {
              id: "32Yqe",
              content: "IDK what i want lol",
              createdAt: "",
              metadata: {},
              type: "other",
            },
          ]}
        ></FeedbackTable>
      </Box>
    </Box>
  );
}

export default Koi;
