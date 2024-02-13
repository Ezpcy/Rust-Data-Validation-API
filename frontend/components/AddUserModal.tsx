"use client";

import React from "react";
import Modal from "./Modal";
import useAddUserModal from "@/hooks/useAddUserModal";
import { FieldValues, SubmitHandler, useForm } from "react-hook-form";
import { toast } from "react-hot-toast";
import { useState } from "react";
import { User } from "@/types";
import { addUser } from "@/actions/addUser";
import Input from "./Input";
import { useUsers } from "@/hooks/useUsers";

const AddUserModal = () => {
  const { addStateUser } = useUsers();
  const [isLoading, setIsLoading] = useState(false);
  const { onClose, isOpen } = useAddUserModal();

  const { register, handleSubmit, reset } = useForm<FieldValues>({
    defaultValues: {
      first_name: "",
      last_name: "",
      age: "",
      pensum: "",
      location: "",
      occupation: "",
      ahv_nr: "",
    },
  });

  const onChange = (open: boolean) => {
    if (!open) {
      reset();
      onClose();
    }
  };

  const onSubmit: SubmitHandler<FieldValues> = async (values) => {
    try {
      setIsLoading(true);

      // Add user
      const newUser: User = {
        first_name: values.first_name,
        last_name: values.last_name,
        age: values.age,
        pensum: values.pensum,
        location: values.location,
        occupation: values.occupation,
        ahv_nr: values.ahv_nr,
      };

      const response = await addUser(newUser);
      console.log(response.status);
      if (response.message === "User successfully created!") {
        const addedUserId = response.id?.["$oid"];
        newUser._id = { $oid: addedUserId };
        addStateUser(newUser);
        toast.success("User added successfully");
        reset();
        onClose();
      } else {
        toast.error(response.message);
      }
    } catch (error: any) {
      toast.error(error.toString());
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div>
      <Modal
        isOpen={isOpen}
        onChange={onChange}
        title="Add User"
        description=""
      >
        <form
          onSubmit={handleSubmit(onSubmit)}
          className="flex flex-col gap-y-4"
        >
          Vorname:
          <Input
            id="first_name"
            disabled={isLoading}
            {...register("first_name", { required: true })}
            placeholder="First name"
          />
          Nachname:
          <Input
            id="last_name"
            disabled={isLoading}
            {...register("last_name", { required: true })}
            placeholder="Last name"
          />
          Alter:
          <Input
            id="age"
            disabled={isLoading}
            {...register("age", { required: true })}
            placeholder="Age"
          />
          Pensum:
          <Input
            id="pensum"
            disabled={isLoading}
            {...register("pensum", { required: true })}
            placeholder="Pensum"
          />
          Ort:
          <Input
            id="location"
            disabled={isLoading}
            {...register("location", { required: true })}
            placeholder="Location"
          />
          Beruf:
          <Input
            id="occupation"
            disabled={isLoading}
            {...register("occupation", { required: true })}
            placeholder="Occupation"
          />
          AHV Nummer:
          <Input
            id="ahv_nr"
            disabled={isLoading}
            {...register("ahv_nr", { required: true })}
            placeholder="AHV Number"
          />
          <div className="flex mt-4">
            <button
              className="
              px-4 
              py-2 
              bg-blue-600 
              text-white 
              rounded-md 
              hover:bg-blue-500 
              focus:outline-none
            "
              type="submit"
              disabled={isLoading}
              onClick={handleSubmit(onSubmit)}
            >
              Add
            </button>
          </div>
        </form>
      </Modal>
    </div>
  );
};

export default AddUserModal;
