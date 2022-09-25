/* eslint-disable @typescript-eslint/no-misused-promises */

import {
  Button,
  ButtonGroup,
  Flex,
  FormControl,
  FormHelperText,
  FormLabel,
  Heading,
  Input,
  Menu,
  MenuButton,
  MenuDivider,
  MenuItem,
  MenuList,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  useDisclosure,
} from "@chakra-ui/react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { Project } from "../../interfaces/project";
import { createProject } from "../../features/project/api";
import { useSession } from "../../state/session";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { getUserProfile } from "../../features/profile/api";

function Header(): JSX.Element {
  // Zustand
  const {
    sessionToken,
    clearSessionToken,
    selectedProjectId,
    setSelectedProjectId,
  } = useSession();

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

  // Profile from react query
  const { data: userProfile } = useQuery(
    ["profile"],
    async () => await getUserProfile(sessionToken),
    { enabled: !!sessionToken }
  );

  // Hook form for creating a new project
  const { handleSubmit, register } = useForm<{ projectName: string }>();

  // For controlling the project selector mode
  const [isSwitchingProject, setIsSwitchingProject] = useState(false);

  function mainMenuItems() {
    return (
      <>
        <MenuItem
          isDisabled
          // isDisabled={!userProfile?.projects.length}
        >
          Settings (WIP)
        </MenuItem>
        <MenuDivider />
        <MenuItem onClick={onOpen}>Create project</MenuItem>
        <MenuItem
          isDisabled={!userProfile?.projects.length}
          closeOnSelect={false}
          onClick={() => setIsSwitchingProject(true)}
        >
          Switch project
        </MenuItem>
      </>
    );
  }

  function switchingProjectMenuItems(projects: Project[]) {
    return (
      <>
        <MenuItem
          closeOnSelect={false}
          onClick={() => setIsSwitchingProject(false)}
        >
          Back
        </MenuItem>
        <MenuDivider />
        {projects.map((p) => (
          <MenuItem onClick={() => setSelectedProjectId(p.id)} key={p.id}>
            {p.name}
          </MenuItem>
        ))}
      </>
    );
  }
  const { isOpen, onOpen, onClose } = useDisclosure();

  async function onSubmit(values: { projectName: string }) {
    projectMutation.mutate({
      projectName: values.projectName,
      token: sessionToken ?? "",
    });
    onClose();
  }

  return (
    <Flex justifyContent="space-between" alignItems="center">
      <ButtonGroup alignItems="center">
        <Heading
          fontWeight={600}
          fontSize={{ base: "xl", sm: "2xl", md: "4xl" }}
          lineHeight={"110%"}
        >
          Koi
        </Heading>
        <Menu onClose={() => setIsSwitchingProject(false)}>
          <MenuButton variant="outline" as={Button}>
            {userProfile?.projects.find((p) => p.id === selectedProjectId)
              ?.name ?? "Create a new project"}
          </MenuButton>
          <MenuList>
            {isSwitchingProject
              ? switchingProjectMenuItems(userProfile?.projects ?? [])
              : mainMenuItems()}
          </MenuList>
        </Menu>
      </ButtonGroup>
      <ButtonGroup>
        <Button onClick={clearSessionToken}>Log Out</Button>
      </ButtonGroup>
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
    </Flex>
  );
}

export { Header };
