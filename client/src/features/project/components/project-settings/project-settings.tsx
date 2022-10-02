import {
  TableContainer,
  Table,
  Thead,
  Tr,
  Th,
  Tbody,
  Td,
  Box,
  Button,
  IconButton,
  useDisclosure,
} from "@chakra-ui/react";
import { CloseIcon, ArrowBackIcon } from "@chakra-ui/icons";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useAppParams } from "../../../../router";
import { useSession } from "../../../../state/session";
import { getUserProfile } from "../../../profile/api";
import { getProjectUsers, removeUserFromProject } from "../../api";
import { useNavigate } from "react-router-dom";
import { AddUserModal } from "./modals/add-user";

function ProjectSettings() {
  // Zustand
  const { sessionToken } = useSession();

  const { projectId } = useAppParams();

  const queryClient = useQueryClient();

  const { data: userProfile } = useQuery(
    ["profile"],
    async () => await getUserProfile(sessionToken),
    { enabled: !!sessionToken }
  );

  const { data: projectUsers } = useQuery(
    ["projectUsers", projectId],
    async () =>
      await getProjectUsers(projectId as string, sessionToken as string),

    { enabled: !!sessionToken && !!projectId }
  );

  const userRemoveMutation = useMutation(
    async (values: {
      projectId: string;
      userToBeRemoved: string;
      token: string;
    }) =>
      await removeUserFromProject(
        values.projectId,
        { email: values.userToBeRemoved },
        values.token
      ),
    {
      onSettled: async (data, variables) => {
        await queryClient.invalidateQueries(["projectUsers"]);
      },
    }
  );

  const navigate = useNavigate();

  const { isOpen, onOpen, onClose } = useDisclosure();

  return (
    <Box>
      <IconButton
        onClick={() => navigate(`/app/${projectId as string}`)}
        marginRight={2}
        aria-label="Back"
        icon={<ArrowBackIcon />}
      />
      <AddUserModal isOpen={isOpen} onClose={onClose} />
      <Button onClick={onOpen}>Add user</Button>
      <TableContainer
        borderStyle="solid"
        borderWidth="1px"
        borderRadius="8px"
        p={2}
        my={4}
        borderColor="gray.200"
      >
        <Table variant="simple">
          <Thead>
            <Tr>
              <Th>Name</Th>
              <Th>Email address</Th>
              <Th>Action</Th>
            </Tr>
          </Thead>
          <Tbody>
            {projectUsers?.map((u) => (
              <Tr key={u.id}>
                <Td>{u.name}</Td>
                <Td>{u.email}</Td>
                <Td>
                  <IconButton
                    size="xs"
                    disabled={u.id === userProfile?.userProfile?.id}
                    icon={<CloseIcon />}
                    onClick={() =>
                      userRemoveMutation.mutate({
                        projectId: projectId as string,
                        userToBeRemoved: u.email,
                        token: sessionToken as string,
                      })
                    }
                    aria-label={"Remove user from project"}
                  />
                </Td>
              </Tr>
            ))}
          </Tbody>
        </Table>
      </TableContainer>
    </Box>
  );
}

export { ProjectSettings };
