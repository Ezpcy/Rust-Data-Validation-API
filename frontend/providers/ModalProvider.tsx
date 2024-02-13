"use client";
import AddUserModal from "@/components/AddUserModal";
import EditUserModal from "@/components/EditUserModal";
import { useState, useEffect } from "react";

const ModalProvider = () => {
  const [isMounted, setIsMounted] = useState(false);

  useEffect(() => {
    setIsMounted(true);
  }, []);

  if (!isMounted) {
    return (
      <div>
        <p>Loading...</p>
      </div>
    );
  }

  return (
    <>
        <AddUserModal />
        <EditUserModal />
    </>
    );
};

export default ModalProvider;
