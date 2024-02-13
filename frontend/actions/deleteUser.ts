const API_URL = process.env.NEXT_PUBLIC_MONGO_API;

export const deleteUser = async (userId: string) => {
  const res = await fetch(`${API_URL}/user/${userId}`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(userId),
  });
  const data = await res.json();
  return data;
};
