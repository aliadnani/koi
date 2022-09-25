/* eslint-disable @typescript-eslint/no-misused-promises */

import { FormControl, FormLabel, Input, Button } from "@chakra-ui/react";
import { useForm } from "react-hook-form";
import shallow from "zustand/shallow";
import { camelCaseObject } from "../../../common/utils";
import { useSessionStore } from "../../../store.ts";
import { registerAccount } from "../api";

function registerForm() {
  const { setSession } = useSessionStore(
    (state) => ({ setSession: state.setSession }),
    shallow
  );

  async function onSubmit(
    values: { name: string; email: string; password: string } | any
  ) {
    const session = await registerAccount(
      values.name,
      values.email,
      values.password
    );
    setSession(camelCaseObject(session));
  }

  const {
    handleSubmit,
    register,
    formState: { errors, isSubmitting },
  } = useForm();

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <FormControl py={1} isInvalid={!!errors.name}>
        <FormLabel htmlFor="name">Full name</FormLabel>
        <Input
          id="name"
          {...register("name", {
            required: "This is required",
          })}
        />
      </FormControl>
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
          id="password"
          type="password"
          {...register("password", {
            required: "This is required",
          })}
        />
      </FormControl>
      <Button mt={4} colorScheme="red" isLoading={isSubmitting} type="submit">
        Submit
      </Button>
    </form>
  );
}

export { registerForm };
