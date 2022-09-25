import {
  Box,
  Accordion,
  AccordionItem,
  AccordionButton,
  AccordionIcon,
  AccordionPanel,
  Tag,
  Text,
  InputGroup,
  Input,
  Button,
  InputRightElement,
} from "@chakra-ui/react";

interface FeedbackItemMetadata {
  email?: string;
  origin?: string;
  device?: string;
}

type FeedbackItemType = "issue" | "idea" | "other";

interface FeedbackItem {
  id: string;
  content: string;
  type: FeedbackItemType;
  createdAt: string;
  metadata: FeedbackItemMetadata;
  additionalAttributes?: { [key: string]: string };
}

interface FeedbackTableProps {
  feedbackItems: FeedbackItem[];
}

function feedbackItemTag(type: FeedbackItemType) {
  switch (type) {
    case "idea": {
      return (
        <Tag bgColor="teal.400" color="white">
          Idea
        </Tag>
      );
    }
    case "issue": {
      return (
        <Tag bgColor="red.400" color="white">
          Issue
        </Tag>
      );
    }
    case "other": {
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
      <Text>
        {`(${props.feedbackItems.length}/${props.feedbackItems.length}) feedback entries `}
      </Text>
      <Accordion defaultIndex={[]} allowToggle>
        {props.feedbackItems.map((fi) => (
          <AccordionItem key={fi.id}>
            <AccordionButton>
              <Box flex="1" textAlign="left">
                {feedbackItemTag(fi.type)}
                <Text>{fi.content}</Text>
              </Box>
              <AccordionIcon />
            </AccordionButton>
            <AccordionPanel pb={4}>
              <Box></Box>
            </AccordionPanel>
          </AccordionItem>
        ))}
      </Accordion>
    </Box>
  );
}

export { FeedbackTable };
