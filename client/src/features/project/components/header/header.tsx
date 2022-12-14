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
import { ReactComponent as KoiLogo } from "../../../../assets/koi.svg";
import { useAppParams } from "../../../../router";
import { useNavigate } from "react-router-dom";

function Header(): JSX.Element {
  // React Router
  const navigate = useNavigate();
  const { projectId } = useAppParams();

  // Zustand state
  const { sessionToken, clearSessionToken } = useSession();

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
          isDisabled={!userProfile?.projects.length}
          onClick={() => navigate(`${projectId as string}/settings`)}
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
          <MenuItem onClick={() => navigate(p.id)} key={p.id}>
            {p.name}
          </MenuItem>
        ))}
      </>
    );
  }

  return (
    <Flex justifyContent="space-between" alignItems="center">
      <ButtonGroup alignItems="center">
        <KoiLogo height="2rem" width="2rem" />
        <Heading
          fontWeight={600}
          fontSize={{ base: "xl", sm: "2xl", md: "4xl" }}
          lineHeight={"110%"}
        >
          Koi
        </Heading>
        <Menu onClose={() => setIsSwitchingProject(false)}>
          <MenuButton variant="outline" as={Button}>
            {userProfile?.projects.find((p) => p.id === projectId)?.name ??
              "Create a new project"}
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
        {projectId && (
          <Button
            onClick={() => {
              navigate(`${projectId}/help`);
            }}
          >
            Help
          </Button>
        )}
        <Button onClick={clearSessionToken}>Log Out</Button>
      </ButtonGroup>
    </Flex>
  );
}

export { Header };
