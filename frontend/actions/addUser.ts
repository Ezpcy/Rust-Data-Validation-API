import { User } from "@/types";

const API_URL = process.env.NEXT_PUBLIC_MONGO_API;

export const addUser = async (user: User) => {
    const res = await fetch(`${API_URL}/user`, {
        method: "POST",
        headers: {
        "Content-Type": "application/json",
        },
        body: JSON.stringify(user),
    });
    
    const data = await res.json();
    return data;
}