"use client";

import { useState, useEffect } from "react";
import SearchBar from "@/components/SearchBar";
import { Table } from "@/components/Table";
import AddButton from "@/components/AddButton";
import { User } from "@/types";
import { useUsers } from "@/hooks/useUsers";
import { getUsers } from "@/actions/getUsers";

const TableProvider = () => {
  const { users, setUsers } = useUsers();
  const [isMounted, setIsMounted] = useState(false);
  const [searchValue, setSearchValue] = useState<string>("");

  const handleSearchValueChange = (value: string) => {
    setSearchValue(value);
  };

  useEffect(() => {
    const fetchUsers = async () => {
      const users = await getUsers();
      setUsers(users);
    };
    fetchUsers();
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
      <AddButton />
      <SearchBar onSearchValueChange={handleSearchValueChange} />
      <Table filterValue={searchValue} users={users} />
    </>
  );
};

export default TableProvider;
