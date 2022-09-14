import { Flex, Heading } from "@chakra-ui/react";

function TopNav() {
  return (
    <Flex justifyContent="space-between">
      <Flex align="flex-start">
        <Heading>KOI</Heading>
      </Flex>
      <Flex align="flex-end">
        <Heading>Login</Heading>
        <Heading>Help</Heading>
        <Heading>Whoa</Heading>
      </Flex>
    </Flex>
  );
}

export default TopNav;
