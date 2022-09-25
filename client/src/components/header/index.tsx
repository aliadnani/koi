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
} from "@chakra-ui/react";
import { useState } from "react";
import { Project } from "../../interfaces/project";

function Header(): JSX.Element {
  const mockProjects = [
    { id: "1", name: "project1", createdAt: "yesterday" },
    { id: "2", name: "project2", createdAt: "yesterday" },
    { id: "3", name: "project3", createdAt: "yesterday" },
  ];

  const [currentProject, setCurrentProject] = useState(mockProjects[0]?.id);
  const [isSwitchingProject, setIsSwitchingProject] = useState(false);

  function mainMenuItems() {
    return (
      <>
        <MenuItem isDisabled={!mockProjects.length}>Settings</MenuItem>
        <MenuDivider />
        <MenuItem>Create project</MenuItem>
        <MenuItem
          isDisabled={!mockProjects.length}
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
          <MenuItem key={p.id}>{p.name}</MenuItem>
        ))}
      </>
    );
  }

  return (
    <Flex justifyContent="space-between" alignItems="center">
      <ButtonGroup alignItems="center">
        <Heading
          fontWeight={600}
          fontSize={{ base: "2xl", sm: "4xl", md: "6xl" }}
          lineHeight={"110%"}
        >
          Koi
        </Heading>
        <Menu>
          <MenuButton variant="outline" as={Button}>
            {mockProjects.find((p) => p.id === currentProject)?.name ??
              "Create a new project"}
          </MenuButton>
          <MenuList>
            {isSwitchingProject
              ? switchingProjectMenuItems(mockProjects)
              : mainMenuItems()}
          </MenuList>
        </Menu>
      </ButtonGroup>
      <ButtonGroup>
        <Button>Log Out</Button>
      </ButtonGroup>
    </Flex>
  );
}

export { Header };
