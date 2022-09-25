import {
  Box,
  Heading,
  Container,
  Text,
  Stack,
  Link,
  Tab,
  TabList,
  TabPanel,
  TabPanels,
  Tabs,
} from "@chakra-ui/react";
import { useSessionStore } from "../../store.ts";
import { logInForm } from "./forms/login";
import { registerForm } from "./forms/register";
import shallow from "zustand/shallow";
import { useNavigate } from "react-router-dom";
import { useEffect } from "react";

function LandingPage() {
  const { session } = useSessionStore(
    (state) => ({ session: state.session }),
    shallow
  );

  const navigate = useNavigate();

  useEffect(() => {
    if (session) {
      navigate("app");
    }
  }, [session]);

  return (
    <Box>
      <Container maxW={"3xl"}>
        <Stack
          as={Box}
          textAlign={"left"}
          spacing={{ base: 4, md: 8 }}
          py={{ base: 4, md: 8 }}
        >
          <Box>
            <Heading
              fontWeight={600}
              fontSize={{ base: "2xl", sm: "4xl", md: "6xl" }}
              lineHeight={"110%"}
            >
              Koi
            </Heading>
            <Heading
              fontWeight={300}
              fontSize={{ base: "xl", sm: "3xl", md: "5xl" }}
              lineHeight={"110%"}
            >
              Lorem ipsum dolor sit amet, consectetur adipiscing elit.
            </Heading>
          </Box>
          <Text color={"gray.500"}>
            Integer vitae fringilla dolor. Interdum et malesuada fames ac ante
            ipsum primis in faucibus. Donec vestibulum scelerisque erat, vitae
            sodales metus dignissim vitae. Vestibulum eu mauris magna.
          </Text>
          <Link isExternal href="https://github.com/aliadnani/koi">
            https://github.com/aliadnani/koi
          </Link>
          <Tabs colorScheme="gray">
            <TabList>
              <Tab>Log In</Tab>
              <Tab>Register</Tab>
            </TabList>

            <TabPanels>
              <TabPanel>{logInForm()}</TabPanel>
              <TabPanel>{registerForm()}</TabPanel>
            </TabPanels>
          </Tabs>
        </Stack>
      </Container>
    </Box>
  );
}

export { LandingPage };
