import {
  Box,
  Accordion,
  AccordionItem,
  AccordionButton,
  AccordionIcon,
  AccordionPanel,
  Tag,
  Text,
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
      return <Tag colorScheme="orange">Idea</Tag>;
    }
    case "issue": {
      return <Tag colorScheme="red">Issue</Tag>;
    }
    case "other": {
      return <Tag colorScheme="blue">Other</Tag>;
    }
    default: {
      return <Tag>Other</Tag>;
    }
  }
}

export function FeedbackTable(props: FeedbackTableProps) {
  return (
    <Accordion defaultIndex={[]} margin="0 auto" maxWidth="800px" padding="1rem" allowToggle>
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
  );
}
