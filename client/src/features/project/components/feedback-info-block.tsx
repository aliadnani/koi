import { Code, ListItem, Text, UnorderedList } from "@chakra-ui/react";

function FeedbackInfoBlock(props: { apiBaseUrl: string; projectId: string }) {
  return (
    <Text>
      Welcome to Koi!
      <br />
      <br />
      You can:
      <UnorderedList>
        <ListItem>
          Create and switch between projects on the top-left dropdown
        </ListItem>
        <ListItem>View and manage collected feedback below</ListItem>
      </UnorderedList>
      <br />
      A UI for collecting feedback is WIP at the moment so you'll need to do it
      manually via API request:
      <br />
      <br />
      <Code
        borderRadius={8}
        p={2}
        overflow="scroll"
        display="block"
        whiteSpace="pre"
      >
        {`curl -X 'POST' '${props.apiBaseUrl}/feedback' \\
  -H 'Accept: application/json' \\
  -H 'Content-Type: application/json' \\
  -d '{
    "project_id": "${props.projectId}",
    "category": "Idea",
    "description": "Hi, please make a UI widget for submitting feedback. Thanks!",
    "location": "${window.location.href}",
    "additional_attributes": {}
  }'`}
      </Code>
      <br />
      <Code>category</Code> can be one of{" "}
      <Code>"Idea" | "Issue" | "Other"</Code>
    </Text>
  );
}

export { FeedbackInfoBlock };
