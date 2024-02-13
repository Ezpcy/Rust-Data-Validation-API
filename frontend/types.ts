export interface User {
    _id?: {
        $oid: string;
      };
    first_name: string;
    last_name: string;
    age: number;
    pensum: number;
    location: string;
    occupation: string;
    ahv_nr: string;

}

export interface newUser {
    _id?: {
        $oid: string;
      };
    first_name?: string;
    last_name?: string;
    age?: number;
    pensum?: number;
    location?: string;
    occupation?: string;
    ahv_nr?: string;
}