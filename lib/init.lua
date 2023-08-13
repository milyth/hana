---@meta
---@class Hana
Hana = {
	---@type function
	---@param path string
	---@return nil
	-- Creates a directory relative to root directory
	makeDirAll = function(path) end,

	---@type function
	---@param path string
	---@param contents string
	---@return nil
	-- Write a file relative to root directory
	writeFile = function(path, contents) end,

	---@type function
	---@param table table
	---@return string
	-- Converts a table into a TOML String
	toToml = function(table) end,

	---@type function
	---@param table table
	---@return string
	-- Converts a table into a Ini string
	toIni = function(table) end,

	---@type function
	---@param name string
	---@return table?
	getUserByName = function(name) end,

	---@type function
	---@param assertion boolean
	---@param why string
	---@return nil
	assert = function(assertion, why) end,
}
