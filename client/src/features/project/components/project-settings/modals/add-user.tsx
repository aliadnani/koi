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
import { useAppParams } from "../../../../../router";
import { useSession } from "../../../../../state/session";
import { addUserToProject } from "../../../api";
  
  function AddUserModal(props: { isOpen: boolean; onClose: () => void }) {
    // Zustand state
    const { sessionToken } = useSession();
    const { projectId } = useAppParams();
  
    // Hook form
    const { handleSubmit, register } = useForm<{ email: string }>();
  
    // React Query
    const queryClient = useQueryClient();
  
    const userAddMutation = useMutation(
        async (values: {
          projectId: string;
          userToBeAdded: string;
          token: string;
        }) =>
          await addUserToProject(
            values.projectId,
            { email: values.userToBeAdded },
            values.token
          ),
        {
          onSettled: async (data, variables) => {
            await queryClient.invalidateQueries(["projectUsers"]);
          },
        }
      );
  
    async function onSubmit(values: { email: string }) {
        userAddMutation.mutate({
        projectId: projectId as string,
        userToBeAdded: values.email,
        token: sessionToken ?? "",
      });
      onClose();
    }
  
    const { isOpen, onClose } = props;
    return (
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Invite a user to this project</ModalHeader>
          <ModalCloseButton />
          <form onSubmit={handleSubmit(onSubmit)}>
            <ModalBody>
              <FormControl isRequired>
                <FormLabel>Email</FormLabel>
                <Input
                  type="text"
                  id="email"
                  {...register("email", {
                    required: "This is required",
                  })}
                />
  
                <FormHelperText>
                  User to be invited must already be registered on Koi!
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
  
  export { AddUserModal };
  