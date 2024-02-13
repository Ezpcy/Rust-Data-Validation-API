import React, { useEffect } from "react";
import Modal from "./Modal";
import useEditUserModal from "@/hooks/useEditUserModal";
import { FieldValues, SubmitHandler, useForm } from "react-hook-form";
import { toast } from "react-hot-toast";
import { useState } from "react";
import { User } from "@/types";
import Input from "./Input";
import { useUsers } from "@/hooks/useUsers";
import { updateUser } from "@/actions/updateUser";

const EditUserModal = () => {
  const { users, updateStateUser } = useUsers();
  const [isLoading, setIsLoading] = useState(false);
  const { onClose, isOpen, user } = useEditUserModal();

  const { register, handleSubmit, setValue } = useForm<FieldValues>();

  useEffect(() => {
    if (user) {
      setValue("first_name", user.first_name);
      setValue("last_name", user.last_name);
      setValue("age", user.age);
      setValue("pensum", user.pensum);
      setValue("location", user.location);
      setValue("occupation", user.occupation);
      setValue("ahv_nr", user.ahv_nr);
    }
  }, [user, setValue]);

  const onChange = (open: boolean) => {
    if (!open) {
      onClose();
    }
  };

  const onOpen = (user: User) => {
    setValue("first_name", user.first_name);
    setValue("last_name", user.last_name);
    setValue("age", user.age);
    setValue("pensum", user.pensum);
    setValue("location", user.location);
    setValue("occupation", user.occupation);
    setValue("ahv_nr", user.ahv_nr);

    onOpen(user);
  };

  const onSubmit: SubmitHandler<FieldValues> = async (values) => {
    try {
      setIsLoading(true);

      // Add user
      const updatedUser: User = {
        _id: user?._id,
        first_name: values?.first_name,
        last_name: values?.last_name,
        age: values?.age,
        pensum: values?.pensum,
        location: values?.location,
        occupation: values?.occupation,
        ahv_nr: values?.ahv_nr,
      };

      const response = await updateUser(updatedUser);
      console.log(response.status);
      if (response.message === "User successfully updated!") {
        updateStateUser(updatedUser);
        toast.success("User updated successfully");
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
        title="Edit User"
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
            {...register("first_name", { required: false })}
          />
          Nachname:
          <Input
            id="last_name"
            disabled={isLoading}
            {...register("last_name", { required: false })}
          />
          Alter:
          <Input
            id="age"
            disabled={isLoading}
            {...register("age", { required: false })}
            placeholder="Age"
          />
          Pensum:
          <Input
            id="pensum"
            disabled={isLoading}
            {...register("pensum", { required: false })}
            placeholder="Pensum"
          />
          Ort:
          <Input
            id="location"
            disabled={isLoading}
            {...register("location", { required: false })}
            placeholder="Location"
          />
          Tätigkeit:
          <Input
            id="occupation"
            disabled={isLoading}
            {...register("occupation", { required: false })}
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
            >
              Add
            </button>
          </div>
        </form>
      </Modal>
    </div>
  );
};

export default EditUserModal;
