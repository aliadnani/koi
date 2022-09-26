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
  Code,
} from "@chakra-ui/react";
import { logInForm } from "./forms/login";
import { registerForm } from "./forms/register";
import { useNavigate } from "react-router-dom";
import { useEffect } from "react";
import { motion } from "framer-motion";
import { useSession } from "../../state/session";

function LandingPage() {
  const { sessionToken } = useSession();

  const navigate = useNavigate();

  useEffect(() => {
    if (sessionToken) {
      navigate("app");
    }
  }, [sessionToken]);

  return (
    <Box>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.5 }}
      >
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
                Get feedback from customers, fast and easily.
              </Heading>
            </Box>
            <Text color={"gray.500"}>
              Still very much in alpha. Do not use for for productive purposes.
              Passwords are <Code>argon2</Code> hashed, but for your own safety,
              do not re-use any personal passwords.
              <br />
              <br />
              Made with <Code>React</Code>, <Code>Rust</Code> &{" "}
              <Code>SQLite</Code> - considering <Code>PostgreSQL</Code> in the
              future!
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
      </motion.div>
    </Box>
  );
}

export { LandingPage };
