import React from 'react';
import { Button } from '@/components/ui/button';
import { RefreshCw, Search, MapPin } from 'lucide-react';

interface HeaderProps {
  city: string;
  onRefresh: () => void;
  onSearch: () => void;
  isLoading: boolean;
}

const Header: React.FC<HeaderProps> = ({ city, onRefresh, onSearch, isLoading }) => {
  return (
    <header className="h-16 glass-dark border-b border-emerald-500/10 flex items-center justify-between px-6">
      <div className="flex items-center space-x-4">
        <div className="flex items-center space-x-2">
          <MapPin className="h-5 w-5 text-emerald-400" />
          <span className="text-white font-medium">{city}</span>
        </div>
      </div>
      
      <div className="flex items-center space-x-3">
        <Button
          variant="ghost"
          size="icon"
          onClick={onSearch}
          className="text-gray-300 hover:text-white hover:bg-emerald-500/10"
        >
          <Search className="h-5 w-5" />
        </Button>
        
        <Button
          variant="ghost"
          size="icon"
          onClick={onRefresh}
          disabled={isLoading}
          className="text-gray-300 hover:text-white hover:bg-emerald-500/10 disabled:opacity-50"
        >
          <RefreshCw className={`h-5 w-5 ${isLoading ? 'animate-spin' : ''}`} />
        </Button>
      </div>
    </header>
  );
};

export default Header;
