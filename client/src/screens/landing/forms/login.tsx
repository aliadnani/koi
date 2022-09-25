/* eslint-disable @typescript-eslint/no-misused-promises */

import { FormControl, FormLabel, Input, Button, Text } from "@chakra-ui/react";
import { useForm } from "react-hook-form";
import shallow from "zustand/shallow";
import { camelCaseObject } from "../../../common/utils";
import { useSessionStore } from "../../../store.ts";
import { logIn } from "../api";

function logInForm() {
  const {
    handleSubmit,
    register,
    setError,
    formState: { errors, isSubmitting },
  } = useForm();
  const { setSession } = useSessionStore(
    (state) => ({ setSession: state.setSession }),
    shallow
  );

  async function onSubmit(
    values: { name: string; email: string; password: string } | any
  ) {
    try {
      const session = await logIn(values.email, values.password);
      setSession(camelCaseObject(session));
    } catch (err) {
      setError("credentials", {
        type: "custom",
        message: "Could not log in",
      });
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <FormControl py={1} isInvalid={!!errors.email}>
        <FormLabel htmlFor="email">Email address</FormLabel>
        <Input
          id="email"
          type="email"
          {...register("email", {
            required: "This is required",
          })}
        />
      </FormControl>
      <FormControl py={1} isInvalid={!!errors.Email}>
        <FormLabel htmlFor="password">Password</FormLabel>
        <Input
          type="password"
          id="password"
          {...register("password", {
            required: "This is required",
          })}
        />
      </FormControl>
      <Button mt={4} colorScheme="red" isLoading={isSubmitting} type="submit">
        Submit
      </Button>
      <Text py={1} color="red">
        {errors.credentials?.message?.toString()}
      </Text>
    </form>
  );
}

export { logInForm };
