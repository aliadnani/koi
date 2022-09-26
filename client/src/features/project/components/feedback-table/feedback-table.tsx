import {
  Box,
  Accordion,
  AccordionItem,
  AccordionButton,
  AccordionIcon,
  AccordionPanel,
  Tag,
  Text,
  Heading,
  Flex,
  ListItem,
  UnorderedList,
  Code,
  Button,
} from "@chakra-ui/react";
import {
  Feedback,
  FeedbackCategory,
  FeedbackStatus,
} from "../../../../interfaces/feedback";

interface FeedbackTableProps {
  feedbackItems: Feedback[];
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function feedbackStatusTag(type: FeedbackStatus) {
  switch (type) {
    case "Default": {
      return (
        <Tag bgColor="green.400" color="white">
          Active
        </Tag>
      );
    }
    case "Archived": {
      return (
        <Tag bgColor="gray.400" color="white">
          Archived
        </Tag>
      );
    }
    default: {
      return <Tag>???</Tag>;
    }
  }
}

function feedbackItemTag(type: FeedbackCategory) {
  switch (type) {
    case "Idea": {
      return (
        <Tag bgColor="teal.400" color="white">
          Idea
        </Tag>
      );
    }
    case "Issue": {
      return (
        <Tag bgColor="red.400" color="white">
          Issue
        </Tag>
      );
    }
    case "Other": {
      return (
        <Tag bgColor="orange.300" color="white">
          Other
        </Tag>
      );
    }
    default: {
      return <Tag>Other</Tag>;
    }
  }
}

function FeedbackTable(props: FeedbackTableProps) {
  return (
    <Box>
      <Heading size="md" marginY={2}>
        {props.feedbackItems.length} feedback entries found.
      </Heading>
      {!props.feedbackItems.length && (
        <Text>Add feedback using the above command.</Text>
      )}
      <Accordion defaultIndex={[]} allowToggle>
        {props.feedbackItems.map((fi) => (
          <AccordionItem key={fi.id}>
            <AccordionButton px={0.5}>
              <Box flex="1" textAlign="left">
                <Flex justifyContent="space-between">
                  {feedbackItemTag(fi.category)}
                  <Text>
                    {new Date(fi.metadata.createdAt).toLocaleString()}
                  </Text>
                </Flex>
                <Text marginY={2}>{fi.description}</Text>
              </Box>
              <AccordionIcon />
            </AccordionButton>
            <AccordionPanel pb={4}>
              <Box>
                <UnorderedList listStyleType="none" marginInlineStart={0}>
                  {/* <ListItem>
                    <Heading size="sm" as="span">
                      Status:{" "}
                    </Heading>
                    {feedbackStatusTag(fi.status)}{" "}
                  </ListItem> */}
                  <ListItem>
                    <Heading size="sm" as="span">
                      Origin:{" "}
                    </Heading>{" "}
                    <Code>{fi.location}</Code>
                  </ListItem>
                  <ListItem>
                    <Heading size="sm" as="span">
                      Device:{" "}
                    </Heading>{" "}
                    <Code>{fi.metadata.device}</Code>
                  </ListItem>
                </UnorderedList>
                <Button disabled float="right" my={2}>
                  Archive (WIP)
                </Button>
              </Box>
            </AccordionPanel>
          </AccordionItem>
        ))}
      </Accordion>
    </Box>
  );
}

export { FeedbackTable };
