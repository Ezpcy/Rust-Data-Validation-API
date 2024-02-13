import { User } from "@/types";
import { create } from "zustand";

interface EditUserModalState {
    user: User | null;
    isOpen: boolean;
    onOpen: (user: User) => void; // Update the onOpen function to accept a User parameter
    onClose: () => void;
}

const useEditUserModal = create<EditUserModalState>((set) => ({
    user: null,
    isOpen: false,
    onOpen: (user: User) => set({ user: user, isOpen: true }), // Set the user state to the passed user value
    onClose: () => set({ isOpen: false }),
}));

export default useEditUserModal;