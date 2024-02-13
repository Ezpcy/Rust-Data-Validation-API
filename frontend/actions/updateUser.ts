import { newUser } from "@/types";

const API_URL = process.env.NEXT_PUBLIC_MONGO_API;

export const updateUser = async (user: newUser) => {
    const res = await fetch(`${API_URL}/user/${user._id?.$oid}`, {
        method: "PUT",
        headers: {
        "Content-Type": "application/json",
        },
        body: JSON.stringify(user),
    });

    const data = await res.json();
    return data;
}