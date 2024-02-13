import { create } from "zustand";

interface AddUserModalState {
    isOpen: boolean;
    onOpen: () => void;
    onClose: () => void;
}

const useAddUserModal = create<AddUserModalState>((set) => ({
    isOpen: false,
    onOpen: () => set({ isOpen: true }),
    onClose: () => set({ isOpen: false }),
}));

export default useAddUserModal;
