import React from 'react';
import { Button } from '@/components/ui/button';
import { Cloud, Wind, Settings, Home } from 'lucide-react';

interface SidebarProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

const Sidebar: React.FC<SidebarProps> = ({ activeTab, onTabChange }) => {
  const tabs = [
    { id: 'dashboard', label: 'Dashboard', icon: Home },
    { id: 'weather', label: 'Weather', icon: Cloud },
    { id: 'air-quality', label: 'Air Quality', icon: Wind },
    { id: 'settings', label: 'Settings', icon: Settings },
  ];

  return (
    <div className="w-64 h-full glass-dark border-r border-white/10 flex flex-col">
      <div className="p-6 border-b border-white/10">
        <h1 className="text-xl font-bold text-white">Weather Dashboard</h1>
        <p className="text-sm text-gray-400">Premium Weather & Air Quality</p>
      </div>
      
      <nav className="flex-1 p-4 space-y-2">
        {tabs.map((tab) => {
          const Icon = tab.icon;
          const isActive = activeTab === tab.id;
          
          return (
            <Button
              key={tab.id}
              variant={isActive ? "default" : "ghost"}
              className={`w-full justify-start space-x-3 h-12 ${
                isActive 
                  ? "bg-white/20 text-white shadow-glow" 
                  : "text-gray-300 hover:bg-white/10 hover:text-white"
              }`}
              onClick={() => onTabChange(tab.id)}
            >
              <Icon className="h-5 w-5" />
              <span className="font-medium">{tab.label}</span>
            </Button>
          );
        })}
      </nav>
      
      <div className="p-4 border-t border-white/10">
        <div className="text-xs text-gray-500 text-center">
          Weather Dashboard v1.0
        </div>
      </div>
    </div>
  );
};

export default Sidebar;
