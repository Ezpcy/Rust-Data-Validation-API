"use client";
import useAddUserModal from "@/hooks/useAddUserModal";
import React from "react";
import { AiOutlinePlus } from "react-icons/ai";


const AddButton = () => {
  const addUserModal = useAddUserModal();

  const onClick = () => {
    addUserModal.onOpen();
  }

  return (
    <div className="">
      <button
      onClick={onClick}
        className="
            bg-blue-900 
            hover:bg-blue-600
            p-2
            pr-3
            rounded-md 
            flex 
            gap-2 
            text-white
        "
      >
        <AiOutlinePlus className="text-red-100 " size={25} />
        Add
      </button>
    </div>
  );
};

export default AddButton;
