"use client";

import useDebounce from "@/hooks/useDebounce";
import React, { useEffect, useState } from "react";

interface SearchBarProps {
    onSearchValueChange: (value: string) => void;
}

const SearchBar = ({ onSearchValueChange }: SearchBarProps) => {
    const [value, setValue] = useState<string>("");
    const debouncedValue = useDebounce<string>(value, 500);
  
    useEffect(() => {
      onSearchValueChange(debouncedValue);
    }, [debouncedValue, onSearchValueChange]);
  
    return (
      <div>
        <input
          className="text-black"
          type="text"
          placeholder="Search"
          value={value}
          onChange={(e) => setValue(e.target.value)}
        />
      </div>
    );
  };
  
  export default SearchBar;

