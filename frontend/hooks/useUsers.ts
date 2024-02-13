import { User, newUser } from '@/types';
import { create } from 'zustand';

type usersState = {
    users: User[];
    setUsers: (users: User[]) => void;
    addStateUser: (user: User) => void;
    deleteStateUser: (id: string) => void;
    updateStateUser: (user: User) => void;
};

export const useUsers = create<usersState>((set) => ({
    users: [],
    setUsers: (users) => set({ users }),
    addStateUser: (user) =>
        set((state) => ({
            users: [...state.users, user],
        })),
    deleteStateUser: (id) =>
        set((state) => ({
            users: state.users.filter((u) => u._id?.$oid !== id),
    })),

    updateStateUser: (user) =>
        set((state) => ({
            users: state.users.map((u) => {
                if (u._id?.$oid === user._id?.$oid) {
                    return user;
                }
                return u;
            }
            ),
        })),
        
}));
