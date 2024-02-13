import { User } from "@/types";
import React, { useState, useEffect } from "react";
import TableRow from "./TableRow";
import { deleteUser } from "@/actions/deleteUser";
import { useUsers } from "@/hooks/useUsers";
import { toast } from "react-hot-toast";

export const revalidate = 0;

interface TableProps {
  filterValue: string;
  users: User[];
}

export const Table: React.FC<TableProps> = ({ filterValue, users }) => {
  // Create a state of an array to push our users
  const [isClient, setIsClient] = useState(false);
  const { deleteStateUser } = useUsers();
  // current page number
  const [currentPage, setCurrentPage] = useState(1);
  // number of items to display per page
  const itemsPerPage = 15;

  const handleDelete = async (userId: string) => {
    const user = userId ?? "";
    if (user) {
      const response = await deleteUser(user);
      if (response.message === "User successfully deleted!") {
        deleteStateUser(user);
        toast.success("User deleted successfully");
      }
    }
  };
    // Filter the users array based on the filterValue prop
    const filteredUsers = users?.filter((user) => {
      const searchValue = filterValue.toLowerCase();
      return (
        user.first_name.toLowerCase().includes(searchValue) ||
        user.last_name.toLowerCase().includes(searchValue) ||
        user.location.toLowerCase().includes(searchValue) ||
        user.occupation.toLowerCase().includes(searchValue) ||
        user.ahv_nr.toLowerCase().includes(searchValue)
      );
    });
    
  // Find the index of the searched user
  const searchedUserIndex = filteredUsers.findIndex((user) => {
    const searchValue = filterValue.toLowerCase();
    return (
      user.first_name.toLowerCase().includes(searchValue) ||
      user.last_name.toLowerCase().includes(searchValue) ||
      user.location.toLowerCase().includes(searchValue) ||
      user.occupation.toLowerCase().includes(searchValue) ||
      user.ahv_nr.toLowerCase().includes(searchValue)
    );
  });
  
  // Calculate the page number of the searched user
  const searchedUserPage = Math.ceil((searchedUserIndex + 1) / itemsPerPage);

  useEffect(() => {
    setIsClient(true);
    setCurrentPage(searchedUserPage);
  }, [users, filterValue, searchedUserPage]);

  if (!isClient) {
    return (
      <>
        <div className="bg-gray-200/20 animate-pulse rounded-md">
          <p>Loading...</p>
        </div>
      </>
    );
  }


  // Calculate the index of the first and last items to display
  const lastIndex = currentPage * itemsPerPage;
  const firstIndex = lastIndex - itemsPerPage;

  // Get the items for the current page
  const currentItems = filteredUsers.reverse().slice(firstIndex, lastIndex);

  // Calculate the total number of pages
  const totalPages = Math.ceil(filteredUsers.length / itemsPerPage);

  // Create an array of page numbers to display
  const pageNumbers = [];
  for (let i = 1; i <= totalPages; i++) {
    pageNumbers.push(i);
  }

  return (
    <div className="relative overflow-x-auto shadow-md sm:rounded-lg">
      <table className="w-full text-sm text-left text-gray-500 dark:text-gray-400">
        <thead className="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
          <tr>
            <th scope="col" className="px-6 py-3">
              First Name
            </th>
            <th scope="col" className="px-6 py-3">
              Last Name
            </th>
            <th scope="col" className="px-6 py-3">
              Location
            </th>
            <th scope="col" className="px-6 py-3">
              Occupation
            </th>
            <th scope="col" className="px-6 py-3">
              AHV Nr.
            </th>
            <th scope="col" className="px-6 py-3">
              Action
            </th>
          </tr>
        </thead>
        <tbody>
          {currentItems.length > 0 ? (
            currentItems
              .map((user) => (
                <TableRow
                  key={user._id?.$oid.toString()}
                  user={user}
                  handleDelete={handleDelete}
                />
              ))
          ) : (
            <div>
              <p>No users found</p>
            </div>
          )}
        </tbody>
      </table>
      <div className="flex justify-between mt-4">
        <button
          onClick={() => setCurrentPage(currentPage - 1)}
          disabled={currentPage === 1}
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 border border-gray-300 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        >
          Previous
        </button>
        <div className="flex">
          {pageNumbers.map((pageNumber) => (
            <button
              key={pageNumber}
              onClick={() => setCurrentPage(pageNumber)}
              className={`mx-1 px-4 py-2 text-sm font-medium rounded-md ${
                pageNumber === currentPage
                  ? "bg-blue-500 text-white"
                  : "bg-gray-100 text-gray-700"
              } hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500`}
            >
              {pageNumber}
            </button>
          ))}
        </div>
        <button
          onClick={() => setCurrentPage(currentPage + 1)}
          disabled={currentPage === totalPages}
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 border border-gray-300 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        >
          Next
        </button>
      </div>
    </div>
  );
};
