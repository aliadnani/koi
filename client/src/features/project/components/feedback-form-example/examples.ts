const exampleForm = (
  projectId: string,
  apiBaseUrl: string
) => `import { Box, Button, Heading, Tab, TabList, Tabs, Textarea } from "@chakra-ui/react";
import ky from "ky";
import { useState } from "react";
import { Globals } from "../../../../common/globals";

function FeedbackFormExample(props: { projectId: string }) {
  type FeedbackType = "Idea" | "Issue" | "Other";

  const [feedbackType, setFeedbackType] = useState < FeedbackType > "Idea";
  const [feedbackBody, setFeedbackBody] = useState("");
  const [feedbackSubmitting, setFeedbackSubmitting] = useState(false);

  async function submitFeedback(type: FeedbackType, body: string) {
    setFeedbackSubmitting(true);
    await ky.post("${apiBaseUrl}/feedback", {
      json: {
        project_id: "${projectId}",
        category: type,
        description: body,
        location: "http://localhost:5173/app",
        additional_attributes: {},
      },
    });
    setFeedbackSubmitting(false);
  }

  function feedbackPlaceholder(f: FeedbackType) {
    switch (f) {
      case "Idea":
        return "Hi, ... would be really cool!";
      case "Issue":
        return "Hi, maybe ... can be improved?";
      case "Other":
        return "Hi, ...";
    }
  }

  return (
    <Box maxW={"320px"} w={"full"} boxShadow={"2xl"} rounded={"lg"} p={6} my={4} textAlign={"left"}>
      <Heading fontSize={"2xl"} my={2} fontFamily={"body"}>
        Talk to us!
      </Heading>

      <Tabs
        variant="soft-rounded"
        my={2}
        onChange={(idx) => {
          setFeedbackBody("");
          switch (idx) {
            case 0:
              setFeedbackType("Idea");
              break;
            case 1:
              setFeedbackType("Issue");
              break;
            case 2:
              setFeedbackType("Other");
              break;
          }
        }}
      >
        <TabList>
          <Tab _selected={{ color: "white", bg: "red.400" }}>Idea</Tab>
          <Tab _selected={{ color: "white", bg: "red.400" }}>Issue</Tab>
          <Tab _selected={{ color: "white", bg: "red.400" }}>Other</Tab>
        </TabList>
      </Tabs>
      <Textarea value={feedbackBody} onChange={(e) => setFeedbackBody(e.target.value)} placeholder={feedbackPlaceholder(feedbackType)} />

      <Button
        flex={1}
        onClick={() => {
          void submitFeedback(feedbackType, feedbackBody);
        }}
        isLoading={feedbackSubmitting}
        my={2}
        fontSize={"sm"}
        rounded={"full"}
        _focus={{
          bg: "gray.200",
        }}
      >
        Submit
      </Button>
    </Box>
  );
}

export { FeedbackFormExample };
`;

const exampleCurl = (
  projectId: string,
  apiBaseUrl: string
) => `curl -X 'POST' '${apiBaseUrl}/feedback' \\
-H 'Accept: application/json' \\
-H 'Content-Type: application/json' \\
-d '{
  "project_id": "${projectId}",
  "category": "Idea",
  "description": "Hi, please make a UI widget for submitting feedback. Thanks!",
  "location": "${window.location.href}",
  "additional_attributes": {}
}'`;

export { exampleCurl, exampleForm };
