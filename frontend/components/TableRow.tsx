import { deleteUser } from "@/actions/deleteUser";
import useEditUserModal from "@/hooks/useEditUserModal";
import { useUsers } from "@/hooks/useUsers";
import { User } from "@/types";
import { useEffect, useState } from "react";

interface TableRowProps {
  user: User;
  handleDelete: (userId: string) => void;
}

const TableRow: React.FC<TableRowProps> = ({ user, handleDelete }) => {
  const { onOpen } = useEditUserModal();
  const { setUsers, deleteStateUser } = useUsers();

  const handleEdit = () => {
    onOpen(user);
  };

  const handleDeleteClick = () => {
    if (user._id?.$oid) {
      handleDelete(user._id.$oid.toString());
    }
  };


  return (
    <>
      <tr
        key={user._id?.$oid.toString()}
        className="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600"
      >
        <th
          scope="row"
          className="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
        >
          {user.first_name}
        </th>
        <td className="px-6 py-4">{user.last_name}</td>
        <td className="px-6 py-4">{user.location}</td>
        <td className="px-6 py-4">{user.occupation}</td>
        <td className="px-6 py-4">{user.ahv_nr}</td>

        <td className="flex items-center px-6 py-4 space-x-3">
          <button
            className="font-medium text-blue-600 dark:text-blue-500 hover:underline"
            onClick={handleEdit}
          >
            Edit
          </button>
          <button
            className="font-medium text-red-600 dark:text-red-500 hover:underline"
            onClick={handleDeleteClick}
          >
            Remove
          </button>
        </td>
      </tr>
    </>
  );
};

export default TableRow;
