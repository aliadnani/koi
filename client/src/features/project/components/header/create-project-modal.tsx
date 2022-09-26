/* eslint-disable @typescript-eslint/no-misused-promises */

import {
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalCloseButton,
  ModalBody,
  FormControl,
  FormLabel,
  Input,
  FormHelperText,
  ModalFooter,
  Button,
} from "@chakra-ui/react";
import { useQueryClient, useMutation } from "@tanstack/react-query";
import { useForm } from "react-hook-form";
import { useSession } from "../../../../state/session";
import { createProject } from "../../api";

function CreateProjectModal(props: { isOpen: boolean; onClose: () => void }) {
  // Zustand state
  const { sessionToken } = useSession();

  // Hook form
  const { handleSubmit, register } = useForm<{ projectName: string }>();

  // React Query
  const queryClient = useQueryClient();

  const projectMutation = useMutation(
    async (values: { projectName: string; token: string }) =>
      await createProject(values.projectName, values.token),
    {
      onSuccess: async (data, variables) => {
        await queryClient.invalidateQueries(["profile"]);
      },
    }
  );

  async function onSubmit(values: { projectName: string }) {
    projectMutation.mutate({
      projectName: values.projectName,
      token: sessionToken ?? "",
    });
    onClose();
  }

  const { isOpen, onClose } = props;
  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>Create a new project</ModalHeader>
        <ModalCloseButton />
        <form onSubmit={handleSubmit(onSubmit)}>
          <ModalBody>
            <FormControl isRequired>
              <FormLabel>Project name</FormLabel>
              <Input
                type="text"
                id="projectName"
                {...register("projectName", {
                  required: "This is required",
                })}
              />

              <FormHelperText>
                Enter the name of your new project
              </FormHelperText>
            </FormControl>
          </ModalBody>

          <ModalFooter>
            <Button
              type="submit"
              onClick={() => {
                onClose();
              }}
              variant="outline"
            >
              Submit
            </Button>
          </ModalFooter>
        </form>
      </ModalContent>
    </Modal>
  );
}

export { CreateProjectModal };
