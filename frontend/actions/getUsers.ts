const API_URL = process.env.NEXT_PUBLIC_MONGO_API;

export const getUsers = async () => {
    try {
        const res = await fetch(`${API_URL}/users`);
        const data = await res.json();
        return data;
    } catch (error) {
        console.log(error);
    }
}