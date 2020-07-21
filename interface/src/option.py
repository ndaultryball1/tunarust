from abc import ABC, abstractmethod

class Option(ABC):

    @abstractmethod
    @property
    def parameters(self):
        """
        Should be able to define pricing parameters
        from python
        """
        pass

    @abstractmethod
    def _price_extern(self):
        """
        This should be a thin wrapper around a call to
        external rust code, and return an array of prices
        """
        pass

    @abstractmethod
    def price(self, spot=None):
        """
        If spot price is not specified, return an array of
        option prices