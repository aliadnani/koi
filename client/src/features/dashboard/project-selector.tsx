import { Menu, MenuButton, Button, MenuList, MenuItem } from "@chakra-ui/react";

interface Project {
  id: string;
  name: string;
}

interface ProjectSelectorProps {
  projects: Project[];
  selectedProjectId: string;
}

export function ProjectSelector(props: ProjectSelectorProps) {
  return (
    <Menu>
      <MenuButton variant="outline" as={Button}>
        {props.projects.find((p) => p.id === props.selectedProjectId)?.name}
      </MenuButton>
      <MenuList>
        {props.projects.map((p) => (
          <MenuItem key={p.id}>{p.name}</MenuItem>
        ))}
      </MenuList>
    </Menu>
  );
}
