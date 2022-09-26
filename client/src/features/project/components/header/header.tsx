/* eslint-disable @typescript-eslint/no-misused-promises */

import {
  Button,
  ButtonGroup,
  Flex,
  Heading,
  Menu,
  MenuButton,
  MenuDivider,
  MenuItem,
  MenuList,
  useDisclosure,
} from "@chakra-ui/react";
import { useState } from "react";
import { Project } from "../../../../interfaces/project";
import { useSession } from "../../../../state/session";
import { useQuery } from "@tanstack/react-query";
import { getUserProfile } from "../../../profile/api";
import { CreateProjectModal } from "./create-project-modal";

function Header(): JSX.Element {
  // Zustand state
  const {
    sessionToken,
    clearSessionToken,
    selectedProjectId,
    setSelectedProjectId,
  } = useSession();

  // React Query state
  const { data: userProfile } = useQuery(
    ["profile"],
    async () => await getUserProfile(sessionToken),
    { enabled: !!sessionToken }
  );

  // For controlling the project selector mode
  const [isSwitchingProject, setIsSwitchingProject] = useState(false);

  // Create project modal
  const { isOpen, onOpen, onClose } = useDisclosure();

  function mainMenuItems() {
    return (
      <>
        <MenuItem
          // TODO: Implement project settings
          isDisabled={!userProfile?.projects.length || true}
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
          <CreateProjectModal isOpen={isOpen} onClose={onClose} />
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
    </Flex>
  );
}

export { Header };
