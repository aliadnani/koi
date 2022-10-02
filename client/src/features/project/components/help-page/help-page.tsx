import {
  Box,
  Button,
  Code,
  Text,
  Heading,
  IconButton,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalHeader,
  ModalOverlay,
  useDisclosure,
} from "@chakra-ui/react";
import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { Globals } from "../../../../common/globals";
import { useAppParams } from "../../../../router";
import { useSession } from "../../../../state/session";
import { getUserProfile } from "../../../profile/api";
import { FeedbackFormExample } from "../feedback-form-example/form";
import { ArrowBackIcon } from "@chakra-ui/icons";
import { exampleCurl, exampleForm } from "../feedback-form-example/examples";

function ProjectHelpPage() {
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

  const { isOpen, onOpen, onClose } = useDisclosure();

  return (
    <>
      <Heading>
        <IconButton
          onClick={() => navigate(-1)}
          marginRight={2}
          aria-label="Back"
          icon={<ArrowBackIcon />}
        />
        Collecting Feedback
      </Heading>
      <Text>
        Koi follows a bring your own UI approach for collecting feedback -
        giving you the flexibility in designing & implementing feedback forms.
        <br />
        <br />
        Here is a React example to get you started:
        <br />
        <br />
        <Box>
          <Modal isOpen={isOpen} onClose={onClose}>
            <ModalOverlay />
            <ModalContent>
              <ModalHeader>Modal Title</ModalHeader>
              <ModalCloseButton />
              <ModalBody>
                <Code
                  borderRadius={8}
                  p={2}
                  overflow="scroll"
                  display="block"
                  whiteSpace="pre"
                >
                  {exampleForm(projectId as string, Globals.apiBaseUrl)}
                </Code>
              </ModalBody>
            </ModalContent>
          </Modal>
          <Button onClick={onOpen}>Show code</Button>
          <FeedbackFormExample projectId={projectId as string} />
        </Box>
        Otherwise, feedback can manually be created via API like so:
        <br />
        <br />
        <Code
          borderRadius={8}
          p={2}
          overflow="scroll"
          display="block"
          whiteSpace="pre"
        >
          {exampleCurl(projectId as string, Globals.apiBaseUrl)}
        </Code>
        <br />
        <Code>category</Code> can be one of{" "}
        <Code>"Idea" | "Issue" | "Other"</Code>
      </Text>
    </>
  );
}

export { ProjectHelpPage };
